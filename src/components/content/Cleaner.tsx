import { createSignal, onMount, onCleanup, For, createMemo } from 'solid-js';
import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { downloadDir, tempDir } from "@tauri-apps/api/path";
import CheckBox from "@components/generic/Checkbox";

type DirKeys = 'temp' | 'tempSystem' | 'downloads';
type Stats = { deleted: number, total: number, freed: number };
type Dirs = Record<DirKeys, string>;

const LABELS = {
  dirs: {
    temp: "Archivos temporales del usuario",
    tempSystem: "Archivos temporales del sistema",
    downloads: "Archivos de descargas"
  },
  stats: {
    deleted: "Archivos eliminados",
    total: "Archivos totales",
    freed: "Espacio liberado"
  }
} as const;

export default function CleanerContent() {
  const [selectedDirs, setSelectedDirs] = createSignal<Record<DirKeys, boolean>>({ temp: true, tempSystem: true, downloads: true });
  const [stats, setStats] = createSignal<Stats>({ deleted: 0, total: 0, freed: 0 });
  const [cleanableDirs, setCleanableDirs] = createSignal<Dirs>({ temp: "", tempSystem: "", downloads: "" });

  const dirsToClean = createMemo(() =>
    Object.entries(selectedDirs())
      .filter(([_, selected]) => selected)
      .map(([dir]) => cleanableDirs()[dir as DirKeys])
  );

  onMount(() => {
    let unlisten: () => void;

    (async () => {
      const [temp, downloads] = await Promise.all([tempDir(), downloadDir()]);
      setCleanableDirs({ temp, tempSystem: `${downloads[0]}:\\Windows\\Temp`, downloads });

      unlisten = await listen<Stats>("cleaner-data", ({ payload }) => setStats(payload));
    })();

    onCleanup(() => {
      if (unlisten) unlisten();
    });
  });

  const runCleaner = async () => {
    const btn = document.getElementById("clean-btn");
    if (!btn) return;
    btn.classList.toggle("btn-disabled");
    await invoke("run_cleaner", { dirs: dirsToClean() });
    btn.classList.toggle("btn-disabled");
  };

  return (
    <>
      <For each={Object.keys(selectedDirs()) as DirKeys[]}>
        {dir => (
          <CheckBox
            text={LABELS.dirs[dir]}
            checked={selectedDirs()[dir]}
            onChange={() => setSelectedDirs(prev => ({ ...prev, [dir]: !prev[dir] }))}
          />
        )}
      </For>
      <div class="lg:flex justify-between items-center mt-4 lg:mt-0">
        <button id="clean-btn" class="btn btn-primary" onClick={runCleaner}>
          Liberar espacio
        </button>
        <div class="stats shadow-lg mt-4 lg:mt-0">
          <For each={Object.entries(LABELS.stats)}>
            {([value, label]) => (
              <div class="stat place-items-center">
                <div class="stat-title">{label}</div>
                <div class="stat-value">{stats()[value as keyof Stats]}</div>
              </div>
            )}
          </For>
        </div>
      </div>
    </>
  );
}
