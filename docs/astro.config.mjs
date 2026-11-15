// @ts-check
import { defineConfig } from 'astro/config';
import starlight from '@astrojs/starlight';

// https://astro.build/config
export default defineConfig({
	site: 'https://ssoj13.github.io',
	base: '/nv-rs',
	integrations: [
		starlight({
			title: 'nv-rs',
			description: 'Direct, honest Rust bindings for the NVIDIA library stack',
			social: [
				{
					icon: 'github',
					label: 'GitHub',
					href: 'https://github.com/ssoj13/nv-rs',
				},
			],
			sidebar: [
				{
					label: 'Getting Started',
					items: [{ autogenerate: { directory: 'getting-started' } }],
				},
				{
					label: 'Core Infrastructure',
					items: [{ autogenerate: { directory: 'core' } }],
				},
				{
					label: 'Interop & Device',
					items: [{ autogenerate: { directory: 'interop' } }],
				},
				{
					label: 'Python API',
					items: [{ autogenerate: { directory: 'python' } }],
				},
				{
					label: 'Crates',
					items: [{ autogenerate: { directory: 'crates' } }],
				},
				{
					label: 'Contributing',
					items: [{ autogenerate: { directory: 'contributing' } }],
				},
			],
		}),
	],
});
