import { invoke } from '@tauri-apps/api';

async function getRunes(exported: { champion: string; role: string; rank: string; region: string }) {
  if ((exported.champion === undefined || exported.champion === '') && exported.role === 'none') {
    return [
      ['Please Enter A Champion Name And Select A Role', null, null, null],
      [null, null],
    ];
  }

  if (exported.champion === undefined || exported.champion === '') {
    return [
      ['Please Enter A Champion Name', null, null, null],
      [null, null],
    ];
  }
  if (exported.role === 'none') {
    return [
      ['Please Select a Role', null, null, null],
      [null, null],
    ];
  }

  try {
    const runes: Array<Array<string | null>> = await invoke('rune_names', {
      name: exported.champion,
      role: exported.role,
      rank: exported.rank,
      region: exported.region,
    });

    return runes;
  } catch (_) {
    return [
      ['No Data Exists!', null, null, null],
      [null, null],
    ];
  }
}

async function getChampionNames() {
  const championNames: string[] = await invoke('champion_names');

  return championNames.map((name) => name.replace(/['"]+/g, ''));
}

export { getRunes, getChampionNames };
