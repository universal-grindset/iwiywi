// iwiywi web pulse — client-side mixer.
//
// The server hands us every PulseItem once. We reimplement the Rust
// PulseMixer's behavior in the browser: dedupe is already done server-side,
// so we only need ordering (random/sequential/by-step/by-source), focus
// filtering, step filtering, and the auto-advance timer. Keeping this
// stateless-on-the-server means the same /api/items response can drive
// many viewers; each browser picks its own cursor, order, and pace.

const state = {
  all: [],           // every PulseItem returned by /api/items
  filtered: [],      // `all` filtered by focus + step
  cursor: 0,
  order: "random",
  focus: "all",
  step: "",
  pulseSecs: 45,
  paused: false,
  date: null,
  timer: null,
  searching: false,
  searchMatches: [],
  searchCursor: 0,
  favorites: JSON.parse(localStorage.getItem("iwiywi_favorites") || "[]"),
};

// Maps source.name() from the Rust side to the Focus variants that admit it.
// Mirrors src/pulse/mod.rs :: Focus::admits — keep in sync if that changes.
const SOURCE_OF_KIND = {
  TodayReading: "today",
  HistoricalReading: "historical",
  BigBookQuote: "big_book",
  Prayer: "prayers",
  StepText: "step_explainers",
  Principle: "step_explainers",
  Tradition: "traditions",
  Concept: "concepts",
  Slogan: "slogans",
  Grapevine: "grapevine",
  Favorite: "favorites",
};

function sourceNameFor(item) {
  return SOURCE_OF_KIND[item.kind] || "";
}

function focusAdmits(focus, item) {
  if (focus === "all") return true;
  const name = sourceNameFor(item);
  if (focus === "steps" || focus === "principles") return name === "step_explainers";
  if (focus === "today") return name === "today";
  if (focus === "history") return name === "historical";
  if (focus === "big_book") return name === "big_book";
  if (focus === "prayers") return name === "prayers";
  if (focus === "grapevine") return name === "grapevine";
  if (focus === "traditions") return name === "traditions";
  if (focus === "concepts") return name === "concepts";
  if (focus === "slogans") return name === "slogans";
  if (focus === "favorites") return name === "favorites";
  return true;
}

function rebuild() {
  let items = state.all.filter((i) => focusAdmits(state.focus, i));
  if (state.step !== "") {
    const want = Number(state.step);
    items = items.filter((i) => i.step === want);
  }
  // Order::ByStep in the Rust mixer drops everything except StepText and
  // sorts 1→12. Mirror that so the web menu feels the same.
  if (state.order === "by-step") {
    items = items
      .filter((i) => i.kind === "StepText")
      .sort((a, b) => (a.step || 255) - (b.step || 255));
  }
  state.filtered = items;
  // Start somewhere random on rebuild so focus changes yield instant visual
  // feedback (new item) rather than "cursor 0 of new filter."
  if (items.length > 0) {
    state.cursor = Math.floor(Math.random() * items.length);
  } else {
    state.cursor = 0;
  }
  render();
}

function current() {
  return state.filtered[state.cursor];
}

function render() {
  const label = document.getElementById("label");
  const body = document.getElementById("body");
  const meta = document.getElementById("meta");
  const main = document.getElementById("pulse");
  const item = current();

  main.classList.add("fading");
  // ~match the CSS transition. The fade keeps the eye calm between items,
  // same spirit as the terminal drift animation.
  window.setTimeout(() => {
    if (!item) {
      label.textContent = "nothing to show";
      body.textContent = state.all.length === 0
        ? "no readings loaded — run `iwiywi fetch` on the server, then reload."
        : "no items match this focus / step.";
    } else {
      label.textContent = item.label;
      body.textContent = item.body;
    }
    const bits = [];
    if (state.date) bits.push(state.date);
    if (state.filtered.length > 0) {
      bits.push(`${state.cursor + 1} / ${state.filtered.length}`);
    }
    bits.push(state.focus + (state.step ? ` · step ${state.step}` : ""));
    if (state.paused) bits.push("paused");
    meta.textContent = bits.join(" · ");
    main.classList.remove("fading");
  }, 180);
}

function advance(delta) {
  if (state.filtered.length === 0) return;
  state.cursor = ((state.cursor + delta) % state.filtered.length + state.filtered.length)
    % state.filtered.length;
  render();
  resetTimer();
}

function randomJump() {
  if (state.filtered.length < 2) {
    render();
    return;
  }
  let next;
  do { next = Math.floor(Math.random() * state.filtered.length); }
  while (next === state.cursor);
  state.cursor = next;
  render();
  resetTimer();
}

function resetTimer() {
  if (state.timer) clearInterval(state.timer);
  state.timer = null;
  if (state.paused || !state.pulseSecs) return;
  state.timer = window.setInterval(() => {
    if (state.order === "random") randomJump();
    else advance(1);
  }, state.pulseSecs * 1000);
}

function setFocusStep(n) {
  state.step = String(n);
  document.getElementById("sel-step").value = state.step;
  rebuild();
}

function clearFocusStep() {
  state.step = "";
  document.getElementById("sel-step").value = "";
  rebuild();
}

function togglePause() {
  state.paused = !state.paused;
  document.getElementById("pause-btn").textContent = state.paused ? "▶" : "⏸";
  toast(state.paused ? "paused" : "playing");
  resetTimer();
  render();
}

function toast(msg) {
  const el = document.getElementById("toast");
  el.textContent = msg;
  el.hidden = false;
  window.clearTimeout(toast._t);
  toast._t = window.setTimeout(() => { el.hidden = true; }, 1500);
}

function wireControls() {
  document.querySelectorAll(".controls button").forEach((btn) => {
    btn.addEventListener("click", () => {
      const act = btn.dataset.act;
      if (act === "next") advance(1);
      else if (act === "prev") advance(-1);
      else if (act === "random") randomJump();
      else if (act === "pause") togglePause();
      else if (act === "fav") toggleFavorite();
      else if (act === "copy") copyCurrentItem();
      else if (act === "search") openSearch();
      else if (act === "menu") document.getElementById("menu").showModal();
    });
  });

  const selOrder = document.getElementById("sel-order");
  const selFocus = document.getElementById("sel-focus");
  const selStep = document.getElementById("sel-step");
  const selSecs = document.getElementById("sel-secs");

  selOrder.addEventListener("change", () => { state.order = selOrder.value; rebuild(); resetTimer(); });
  selFocus.addEventListener("change", () => { state.focus = selFocus.value; rebuild(); });
  selStep.addEventListener("change", () => { state.step = selStep.value; rebuild(); });
  selSecs.addEventListener("change", () => {
    const n = Math.max(0, Number(selSecs.value) || 0);
    state.pulseSecs = n;
    resetTimer();
  });
}

// Matches the TUI's key map so muscle memory transfers. See src/tui/mod.rs.
function wireKeys() {
  window.addEventListener("keydown", (e) => {
    // Don't hijack typing inside the menu dialog.
    if (document.getElementById("menu").open) return;
    if (e.metaKey || e.ctrlKey || e.altKey) return;
    switch (e.key) {
      case "n": advance(1); break;
      case "p": advance(-1); break;
      case "r": randomJump(); break;
      case " ": e.preventDefault(); togglePause(); break;
      case "m": document.getElementById("menu").showModal(); break;
      case "1": case "2": case "3": case "4": case "5":
      case "6": case "7": case "8": case "9":
        setFocusStep(Number(e.key)); break;
      case "0": setFocusStep(10); break;
      case "-": setFocusStep(11); break;
      case "=": setFocusStep(12); break;
      case "*": clearFocusStep(); break;
      case "c": copyCurrentItem(); break;
      case "f": toggleFavorite(); break;
      case "/": e.preventDefault(); openSearch(); break;
      case "Escape": closeSearch(); state.searchMatches = []; break;
      default: return;
    }
  });
}

async function load() {
  const resp = await fetch("/api/items", { cache: "no-store" });
  if (!resp.ok) throw new Error("fetch /api/items failed: " + resp.status);
  const data = await resp.json();
  state.all = data.items || [];
  state.date = data.date;
  if (typeof data.pulse_secs === "number") state.pulseSecs = data.pulse_secs;
  if (data.pulse_secs === null) state.pulseSecs = 0;   // manual-only
  document.getElementById("sel-secs").value = state.pulseSecs;
  rebuild();
  resetTimer();
}

// Every 10 minutes, re-check the server for a new day. The 6am fetch job on
// the host flips `date` and rotates today's readings; this avoids needing a
// manual reload for someone who leaves the page open overnight.
function startDayWatcher() {
  window.setInterval(async () => {
    try {
      const resp = await fetch("/api/items", { cache: "no-store" });
      if (!resp.ok) return;
      const data = await resp.json();
      if (data.date && data.date !== state.date) {
        state.date = data.date;
        state.all = data.items || [];
        rebuild();
        toast("new day — readings refreshed");
      }
    } catch { /* offline — ignore, try again later */ }
  }, 10 * 60 * 1000);
}

// ---------- Fuzzy search (port of Rust fuzzy_score) ----------
function fuzzyScore(haystack, query) {
  if (!query) return 0;
  const hay = haystack.toLowerCase();
  const needle = query.toLowerCase();
  let hi = 0, qi = 0, score = 0, consecutive = 0, firstMatch = -1;
  while (hi < hay.length && qi < needle.length) {
    if (hay[hi] === needle[qi]) {
      consecutive++;
      score += 10 + consecutive * 5;
      if (hi === 0 || !/[a-z0-9]/.test(hay[hi - 1])) score += 30;
      if (firstMatch < 0) firstMatch = hi;
      qi++;
    } else {
      consecutive = 0;
      score--;
    }
    hi++;
  }
  if (qi < needle.length) return null;
  if (firstMatch >= 0 && firstMatch < 8) score += (8 - firstMatch) * 2;
  return score;
}

function openSearch() {
  state.searching = true;
  const bar = document.getElementById("search-bar");
  const input = document.getElementById("search-input");
  bar.hidden = false;
  input.value = "";
  input.focus();
}

function closeSearch() {
  state.searching = false;
  document.getElementById("search-bar").hidden = true;
  document.getElementById("search-count").textContent = "";
}

function runSearch(query) {
  if (!query.trim()) { state.searchMatches = []; return; }
  const scored = state.filtered
    .map((item, i) => ({ s: fuzzyScore(item.label + " " + item.body, query), i }))
    .filter(x => x.s !== null)
    .sort((a, b) => b.s - a.s);
  state.searchMatches = scored.map(x => x.i);
  state.searchCursor = 0;
  document.getElementById("search-count").textContent =
    state.searchMatches.length + " match" + (state.searchMatches.length === 1 ? "" : "es");
}

function jumpToMatch(delta) {
  if (!state.searchMatches.length) return;
  state.searchCursor = (state.searchCursor + delta + state.searchMatches.length) % state.searchMatches.length;
  state.cursor = state.searchMatches[state.searchCursor];
  render();
}

// ---------- Favorites ----------
function isFavorite(item) {
  return state.favorites.some(f => f.label === item.label && f.body === item.body);
}

function toggleFavorite() {
  const item = state.filtered[state.cursor];
  if (!item) return;
  const idx = state.favorites.findIndex(f => f.label === item.label && f.body === item.body);
  if (idx >= 0) {
    state.favorites.splice(idx, 1);
    toast("removed");
  } else {
    state.favorites.push({ label: item.label, body: item.body });
    toast("saved");
  }
  localStorage.setItem("iwiywi_favorites", JSON.stringify(state.favorites));
  updateFavBtn();
}

function updateFavBtn() {
  const item = state.filtered[state.cursor];
  const btn = document.getElementById("fav-btn");
  if (btn) btn.textContent = item && isFavorite(item) ? "\u2605" : "\u2606";
}

// ---------- Copy ----------
function copyCurrentItem() {
  const item = state.filtered[state.cursor];
  if (!item) return;
  const text = item.label + "\n\n" + item.body + "\n";
  if (navigator.clipboard) {
    navigator.clipboard.writeText(text).then(() => toast("copied")).catch(() => toast("copy failed"));
  } else {
    toast("clipboard not available");
  }
}

// Wire search input
document.getElementById("search-input").addEventListener("input", (e) => {
  runSearch(e.target.value);
  if (state.searchMatches.length) {
    state.cursor = state.searchMatches[0];
    render();
  }
});
document.getElementById("search-input").addEventListener("keydown", (e) => {
  if (e.key === "Enter") { e.preventDefault(); closeSearch(); }
  if (e.key === "Escape") { e.preventDefault(); closeSearch(); state.searchMatches = []; }
  if (e.key === "ArrowDown") { e.preventDefault(); jumpToMatch(1); }
  if (e.key === "ArrowUp") { e.preventDefault(); jumpToMatch(-1); }
});

wireControls();
wireKeys();
load().catch((e) => {
  document.getElementById("body").textContent = "couldn't load pulse: " + e.message;
});
startDayWatcher();
