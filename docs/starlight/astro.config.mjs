import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

export default defineConfig({
  integrations: [
    starlight({
      title: 'iwiywi Documentation',
      description: 'Daily AA readings in your terminal',
    }),
  ],
});
