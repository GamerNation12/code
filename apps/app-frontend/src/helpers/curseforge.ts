import { invoke } from '@tauri-apps/api/core'

export interface CurseForgeMod {
	id: number
	name: string
	summary: string
	logo?: { thumbnail_url: string; url: string }
	authors: { name: string; url: string }[]
	download_count: number
	date_modified: string
	categories: { id: number; name: string; url: string }[]
}

export interface CurseForgeFile {
	id: number
	mod_id: number
	display_name: string
	file_name: string
	download_url: string | null
	file_date: string
	game_versions: string[]
	dependencies: { mod_id: number; relation_type: number }[]
}

export async function search_cf(
	query: string,
	class_id?: number,
	game_version?: string,
	mod_loader_type?: number,
	page?: number,
) {
	return await invoke<any>('plugin:curseforge|search_curseforge', {
		query,
		classId: class_id,
		gameVersion: game_version,
		modLoaderType: mod_loader_type,
		page,
	})
}

export async function get_mod_cf(modId: number) {
	return await invoke<CurseForgeMod>('plugin:curseforge|get_mod_curseforge', { modId })
}

export async function get_mod_files_cf(
	modId: number,
	gameVersion?: string,
	modLoaderType?: number,
) {
	return await invoke<CurseForgeFile[]>('plugin:curseforge|get_mod_files_curseforge', {
		modId,
		gameVersion,
		modLoaderType,
	})
}
