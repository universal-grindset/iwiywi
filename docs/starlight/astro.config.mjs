import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'iwiywi',
      description: 'Daily AA readings in your terminal',
      favicon: '/favicon.ico',
      social: {
        github: 'https://github.com/universal-grindset/iwiywi',
      },
      sidebar: [
        {
          label: 'Getting Started',
          items: [
            { label: 'Installation', slug: 'getting-started' },
          ],
        },
        {
          label: 'CLI Reference',
          items: [
            { label: 'Overview', slug: 'cli/index' },
            { label: 'iwiywi (TUI)', slug: 'cli/iwiywi' },
            { label: 'fetch', slug: 'cli/fetch' },
            { label: 'install', slug: 'cli/install' },
          ],
        },
        {
          label: 'Guides',
          items: [
            { label: 'Your First Day', slug: 'guides/first-day' },
            { label: 'Mobile Access', slug: 'guides/mobile-access' },
            { label: 'Manual Updates', slug: 'guides/manual-updates' },
          ],
        },
        {
          label: 'How It Works',
          items: [
            { label: 'Overview', slug: 'how-it-works/index' },
            { label: 'Architecture', slug: 'how-it-works/architecture' },
            { label: 'Reading Sources', slug: 'how-it-works/sources' },
            { label: 'AI Classification', slug: 'how-it-works/classification' },
            { label: 'Vercel Deployment', slug: 'how-it-works/deployment' },
            { label: 'Daily Schedule', slug: 'how-it-works/schedule' },
          ],
        },
        {
          label: 'Troubleshooting',
          items: [
            { label: 'FAQ', slug: 'troubleshooting/index' },
          ],
        },
      ],
      customCss: [],
    }),
  ],
});
