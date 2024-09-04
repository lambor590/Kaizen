import CheckBox from "@/components/generic/Checkbox";
import { invoke } from "@tauri-apps/api";
import { downloadDir } from "@tauri-apps/api/path";
import { tempdir } from "@tauri-apps/api/os";
import { useState, useEffect } from "preact/hooks";
import { listen } from "@tauri-apps/api/event";

type EventData = {
  deleted_files: number;
  total_files: number;
  freed_space: number;
};

type DirType = "temp" | "tempSystem" | "downloads";

export default function Cleaner() {
  const [dirs, setDirs] = useState({ temp: "", downloads: "" });
  const [stats, setStats] = useState({ deleted: 0, total: 0, freed: 0 });
  const [selectedDirs, setSelectedDirs] = useState({ temp: true, tempSystem: true, downloads: true });

  useEffect(() => {
    Promise.all([tempdir(), downloadDir()]).then(([temp, downloads]) =>
      setDirs({ temp, downloads })
    );

    const unlisten = listen("cleaner-data", (event) => {
      const { deleted_files, total_files, freed_space } = event.payload as EventData;
      setStats({ deleted: deleted_files, total: total_files, freed: freed_space });
    });

    return () => {
      unlisten.then(unsub => unsub());
    };
  }, []);

  const cleanableDirs = {
    temp: dirs.temp,
    tempSystem: `${dirs.downloads.charAt(0)}:\\Windows\\Temp`,
    downloads: dirs.downloads
  };

  const handleCheckboxChange = (dir: DirType) =>
    setSelectedDirs(prev => ({ ...prev, [dir]: !prev[dir] }));

  const runCleaner = async () => {
    const btn = document.querySelector("#button");
    btn?.classList.add("btn-disabled");

    const dirsToClean = Object.entries(selectedDirs)
      .filter(([_, isSelected]) => isSelected)
      .map(([dir]) => cleanableDirs[dir as DirType]);

    await invoke("plugin:tools|run_cleaner", { dirs: dirsToClean });
    btn?.classList.remove("btn-disabled");
  };

  return (
    <div className="prose">
      <h1>Liberador de espacio</h1>
      <p>Libera espacio borrando archivos residuales de tu equipo automáticamente.</p>
      <h4>Selecciona los archivos que se van a borrar</h4>
      {Object.entries(selectedDirs).map(([dir, checked]) => (
        <CheckBox
          key={dir}
          text={`Archivos ${dir === 'temp' ? 'temporales del usuario' : dir === 'tempSystem' ? 'temporales del sistema' : 'de descargas'}`}
          checked={checked}
          onChange={() => handleCheckboxChange(dir as DirType)}
        />
      ))}
      <div className="lg:flex justify-between items-center mt-4 lg:mt-0">
        <button id="button" className="btn btn-primary" onClick={runCleaner}>Liberar espacio</button>
        <div className="stats shadow-lg mt-4 lg:mt-0">
          {Object.entries(stats).map(([key, value]) => (
            <div key={key} className="stat place-items-center">
              <div className="stat-title">{key === 'deleted' ? 'Archivos eliminados' : key === 'total' ? 'Archivos totales' : 'Espacio liberado'}</div>
              <div className="stat-value">{value}</div>
            </div>
          ))}
        </div>
      </div>
    </div>
  );
}