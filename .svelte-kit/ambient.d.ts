
// this file is generated — do not edit it


/// <reference types="@sveltejs/kit" />

/**
 * Environment variables [loaded by Vite](https://vitejs.dev/guide/env-and-mode.html#env-files) from `.env` files and `process.env`. Like [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), this module cannot be imported into client-side code. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * _Unlike_ [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), the values exported from this module are statically injected into your bundle at build time, enabling optimisations like dead code elimination.
 * 
 * ```ts
 * import { API_KEY } from '$env/static/private';
 * ```
 * 
 * Note that all environment variables referenced in your code should be declared (for example in an `.env` file), even if they don't have a value until the app is deployed:
 * 
 * ```
 * MY_FEATURE_FLAG=""
 * ```
 * 
 * You can override `.env` values from the command line like so:
 * 
 * ```bash
 * MY_FEATURE_FLAG="enabled" npm run dev
 * ```
 */
declare module '$env/static/private' {
	export const KDE_FULL_SESSION: string;
	export const PROFILEHOME: string;
	export const LANGUAGE: string;
	export const OCIO: string;
	export const PAM_KWALLET5_LOGIN: string;
	export const USER: string;
	export const npm_config_user_agent: string;
	export const TAURI_DEBUG: string;
	export const XDG_SEAT: string;
	export const SSH_AGENT_PID: string;
	export const XDG_SESSION_TYPE: string;
	export const npm_node_execpath: string;
	export const KONSOLE_VERSION: string;
	export const SHLVL: string;
	export const TAURI_PLATFORM_VERSION: string;
	export const XCURSOR_SIZE: string;
	export const npm_config_noproxy: string;
	export const HOME: string;
	export const KDE_APPLICATIONS_AS_SCOPE: string;
	export const LESS: string;
	export const OLDPWD: string;
	export const DESKTOP_SESSION: string;
	export const npm_package_json: string;
	export const LSCOLORS: string;
	export const TAURI_ARCH: string;
	export const ZSH: string;
	export const GTK_RC_FILES: string;
	export const KDE_SESSION_VERSION: string;
	export const PAGER: string;
	export const SHELL_SESSION_ID: string;
	export const XDG_SEAT_PATH: string;
	export const KONSOLE_DBUS_SESSION: string;
	export const MANAGERPID: string;
	export const npm_config_local_prefix: string;
	export const npm_config_userconfig: string;
	export const DBUS_SESSION_BUS_ADDRESS: string;
	export const P9K_TTY: string;
	export const SYSTEMD_EXEC_PID: string;
	export const TAURI_PLATFORM_TYPE: string;
	export const COLOR: string;
	export const COLORTERM: string;
	export const IM_CONFIG_PHASE: string;
	export const LOGNAME: string;
	export const JOURNAL_STREAM: string;
	export const QT_AUTO_SCREEN_SCALE_FACTOR: string;
	export const TAURI_FAMILY: string;
	export const WINDOWID: string;
	export const _: string;
	export const _P9K_SSH_TTY: string;
	export const npm_config_npm_version: string;
	export const npm_config_prefix: string;
	export const COLORFGBG: string;
	export const XDG_SESSION_CLASS: string;
	export const TERM: string;
	export const XDG_SESSION_ID: string;
	export const npm_config_cache: string;
	export const GTK2_RC_FILES: string;
	export const npm_config_node_gyp: string;
	export const PATH: string;
	export const INVOCATION_ID: string;
	export const NODE: string;
	export const OCIOV2: string;
	export const SESSION_MANAGER: string;
	export const npm_package_name: string;
	export const XCURSOR_THEME: string;
	export const XDG_RUNTIME_DIR: string;
	export const XDG_SESSION_PATH: string;
	export const DISPLAY: string;
	export const TAURI_PLATFORM: string;
	export const LANG: string;
	export const MACOSX_DEPLOYMENT_TARGET: string;
	export const XDG_CURRENT_DESKTOP: string;
	export const LS_COLORS: string;
	export const XAUTHORITY: string;
	export const XDG_SESSION_DESKTOP: string;
	export const npm_lifecycle_script: string;
	export const SSH_AUTH_SOCK: string;
	export const SHELL: string;
	export const npm_lifecycle_event: string;
	export const npm_package_version: string;
	export const QT_ACCESSIBILITY: string;
	export const KONSOLE_DBUS_SERVICE: string;
	export const GPG_AGENT_INFO: string;
	export const P9K_SSH: string;
	export const REAL_DEBRID_API_TOKEN: string;
	export const XDG_VTNR: string;
	export const npm_config_globalconfig: string;
	export const npm_config_init_module: string;
	export const PWD: string;
	export const npm_execpath: string;
	export const TAURI_TARGET_TRIPLE: string;
	export const XDG_CONFIG_DIRS: string;
	export const XDG_DATA_DIRS: string;
	export const _P9K_TTY: string;
	export const npm_config_global_prefix: string;
	export const QTWEBENGINE_DICTIONARIES_PATH: string;
	export const npm_command: string;
	export const KDE_SESSION_UID: string;
	export const EDITOR: string;
	export const INIT_CWD: string;
	export const NODE_ENV: string;
}

/**
 * Similar to [`$env/static/private`](https://kit.svelte.dev/docs/modules#$env-static-private), except that it only includes environment variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Values are replaced statically at build time.
 * 
 * ```ts
 * import { PUBLIC_BASE_URL } from '$env/static/public';
 * ```
 */
declare module '$env/static/public' {
	
}

/**
 * This module provides access to runtime environment variables, as defined by the platform you're running on. For example if you're using [`adapter-node`](https://github.com/sveltejs/kit/tree/master/packages/adapter-node) (or running [`vite preview`](https://kit.svelte.dev/docs/cli)), this is equivalent to `process.env`. This module only includes variables that _do not_ begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) _and do_ start with [`config.kit.env.privatePrefix`](https://kit.svelte.dev/docs/configuration#env) (if configured).
 * 
 * This module cannot be imported into client-side code.
 * 
 * ```ts
 * import { env } from '$env/dynamic/private';
 * console.log(env.DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 * 
 * > In `dev`, `$env/dynamic` always includes environment variables from `.env`. In `prod`, this behavior will depend on your adapter.
 */
declare module '$env/dynamic/private' {
	export const env: {
		KDE_FULL_SESSION: string;
		PROFILEHOME: string;
		LANGUAGE: string;
		OCIO: string;
		PAM_KWALLET5_LOGIN: string;
		USER: string;
		npm_config_user_agent: string;
		TAURI_DEBUG: string;
		XDG_SEAT: string;
		SSH_AGENT_PID: string;
		XDG_SESSION_TYPE: string;
		npm_node_execpath: string;
		KONSOLE_VERSION: string;
		SHLVL: string;
		TAURI_PLATFORM_VERSION: string;
		XCURSOR_SIZE: string;
		npm_config_noproxy: string;
		HOME: string;
		KDE_APPLICATIONS_AS_SCOPE: string;
		LESS: string;
		OLDPWD: string;
		DESKTOP_SESSION: string;
		npm_package_json: string;
		LSCOLORS: string;
		TAURI_ARCH: string;
		ZSH: string;
		GTK_RC_FILES: string;
		KDE_SESSION_VERSION: string;
		PAGER: string;
		SHELL_SESSION_ID: string;
		XDG_SEAT_PATH: string;
		KONSOLE_DBUS_SESSION: string;
		MANAGERPID: string;
		npm_config_local_prefix: string;
		npm_config_userconfig: string;
		DBUS_SESSION_BUS_ADDRESS: string;
		P9K_TTY: string;
		SYSTEMD_EXEC_PID: string;
		TAURI_PLATFORM_TYPE: string;
		COLOR: string;
		COLORTERM: string;
		IM_CONFIG_PHASE: string;
		LOGNAME: string;
		JOURNAL_STREAM: string;
		QT_AUTO_SCREEN_SCALE_FACTOR: string;
		TAURI_FAMILY: string;
		WINDOWID: string;
		_: string;
		_P9K_SSH_TTY: string;
		npm_config_npm_version: string;
		npm_config_prefix: string;
		COLORFGBG: string;
		XDG_SESSION_CLASS: string;
		TERM: string;
		XDG_SESSION_ID: string;
		npm_config_cache: string;
		GTK2_RC_FILES: string;
		npm_config_node_gyp: string;
		PATH: string;
		INVOCATION_ID: string;
		NODE: string;
		OCIOV2: string;
		SESSION_MANAGER: string;
		npm_package_name: string;
		XCURSOR_THEME: string;
		XDG_RUNTIME_DIR: string;
		XDG_SESSION_PATH: string;
		DISPLAY: string;
		TAURI_PLATFORM: string;
		LANG: string;
		MACOSX_DEPLOYMENT_TARGET: string;
		XDG_CURRENT_DESKTOP: string;
		LS_COLORS: string;
		XAUTHORITY: string;
		XDG_SESSION_DESKTOP: string;
		npm_lifecycle_script: string;
		SSH_AUTH_SOCK: string;
		SHELL: string;
		npm_lifecycle_event: string;
		npm_package_version: string;
		QT_ACCESSIBILITY: string;
		KONSOLE_DBUS_SERVICE: string;
		GPG_AGENT_INFO: string;
		P9K_SSH: string;
		REAL_DEBRID_API_TOKEN: string;
		XDG_VTNR: string;
		npm_config_globalconfig: string;
		npm_config_init_module: string;
		PWD: string;
		npm_execpath: string;
		TAURI_TARGET_TRIPLE: string;
		XDG_CONFIG_DIRS: string;
		XDG_DATA_DIRS: string;
		_P9K_TTY: string;
		npm_config_global_prefix: string;
		QTWEBENGINE_DICTIONARIES_PATH: string;
		npm_command: string;
		KDE_SESSION_UID: string;
		EDITOR: string;
		INIT_CWD: string;
		NODE_ENV: string;
		[key: `PUBLIC_${string}`]: undefined;
		[key: `${string}`]: string | undefined;
	}
}

/**
 * Similar to [`$env/dynamic/private`](https://kit.svelte.dev/docs/modules#$env-dynamic-private), but only includes variables that begin with [`config.kit.env.publicPrefix`](https://kit.svelte.dev/docs/configuration#env) (which defaults to `PUBLIC_`), and can therefore safely be exposed to client-side code.
 * 
 * Note that public dynamic environment variables must all be sent from the server to the client, causing larger network requests — when possible, use `$env/static/public` instead.
 * 
 * ```ts
 * import { env } from '$env/dynamic/public';
 * console.log(env.PUBLIC_DEPLOYMENT_SPECIFIC_VARIABLE);
 * ```
 */
declare module '$env/dynamic/public' {
	export const env: {
		[key: `PUBLIC_${string}`]: string | undefined;
	}
}
