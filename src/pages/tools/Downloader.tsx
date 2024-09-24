import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "preact/hooks";
import { open } from "@tauri-apps/api/dialog";
import CheckBox from "@/components/generic/Checkbox";
import Label from "@/components/generic/Label";

type VideoData = {
  title: string;
  thumbnail: string;
  duration_string: string;
  channel: string;
  uploader: string;
  original_url: string;
};

export default function Downloader() {
  const [config, setConfig] = useState({ video_url: "", format: "video", quality: "best", output_folder: "", pitch: 1, preserveDuration: true });
  const [videoData, setVideoData] = useState<VideoData>();
  const [depsInstalled, setDepsInstalled] = useState(true);

  useEffect(() => {
    invoke<boolean>("plugin:tools|check_downloader_deps").then(setDepsInstalled);
  }, []);

  const handleChange = async (e: Event) => {
    const { name, value } = e.target as HTMLInputElement;
    setConfig(prev => ({ ...prev, [name]: value }));

    if (name === "video_url" && (!videoData || value !== videoData.original_url)) {
      const dataJson = await invoke<string>("plugin:tools|get_video_data", { url: value });
      setVideoData(dataJson ? JSON.parse(dataJson) : null);
    }
  };

  const selectFolder = async () => {
    const selected = await open({ directory: true, title: "Selecciona la carpeta donde se descargarán los vídeos/audios" });
    if (selected && typeof selected === "string") {
      setConfig(prev => ({ ...prev, output_folder: selected }));
      (document.getElementById("folder-path") as HTMLInputElement).value = selected;
    }
  }

  const performAction = async (action: () => Promise<void>, buttonId: string) => {
    const button = document.getElementById(buttonId) as HTMLButtonElement;
    button.classList.add("btn-disabled");
    await action();
    button.classList.remove("btn-disabled");
  };

  const installDeps = () => performAction(async () => {
    await invoke("plugin:tools|install_downloader_deps");
    setDepsInstalled(true);
  }, 'install-deps-button');

  const runDownloader = () => performAction(async () => {
    if (config.video_url && config.output_folder) {
      await invoke("plugin:tools|run_downloader", { config });
    }
  }, 'run-downloader-button');

  return (
    <div className="prose">
      <h1>Descarga de vídeos y audios</h1>
      <p>Descarga vídeos y audios de más de 1.000 plataformas diferentes.</p>
      <small className="text-warning">
        Esta herramienta está en constante desarrollo y es experimental.<br />
        Algunas cosas mejorarán con el tiempo y se incluirán nuevas características.<br />
        Puedes hacer sugerencias en Discord.
      </small>
      {!depsInstalled ?
        <>
          <p>
            No tienes los programas necesarios para descargar contenido.<br />
            Presiona este botón para descargarlos automáticamente donde está instalado Kaizen.<br />
            Solo tomará unos segundos.
          </p>
          <button id="install-deps-button" className="btn btn-primary" onClick={installDeps}>Descargar</button>
        </>
        :
        <>
          <div className="grid grid-cols-2 gap-8 mb-10">
            <Label text="URL">
              <input type="text" placeholder="https://youtu.be/dQw4w9WgXcQ" name="video_url" className="input input-bordered w-full" onFocusOut={handleChange} />
            </Label>
            <Label text="Carpeta de destino">
              <input type="text" onClick={selectFolder} id="folder-path" placeholder="Seleccionar carpeta de destino" className="input input-bordered w-full cursor-pointer" readOnly />
            </Label>
            <Label text="Formato">
              <select name="format" className="select select-bordered w-full" onChange={handleChange}>
                <option value="video">Solo vídeo (si está disponible)</option>
                <option value="audio">Solo audio</option>
                <option value="both">Vídeo y audio</option>
              </select>
            </Label>
            <Label text="Calidad">
              <select name="quality" className="select select-bordered w-full" onChange={handleChange}>
                <option value="best">Mejor</option>
              </select>
            </Label>
            <Label text="Tono">
              <select name="pitch" className="select select-bordered w-full" onChange={handleChange}>
                <option value={1.5} >Muy agudo</option>
                <option value={1.3} >Agudo</option>
                <option value={1.1} >Un poco agudo</option>
                <option value={1} selected >Normal</option>
                <option value={0.9}>Un poco grave</option>
                <option value={0.7}>Grave</option>
                <option value={0.5}>Muy grave</option>
              </select>
            </Label>
            <CheckBox text="Conservar duración" checked={config.preserveDuration} onChange={() => setConfig(prev => ({ ...prev, remux: !prev.preserveDuration }))} />
          </div>
          {videoData && (
            <div className="grid grid-cols-2 gap-8 mb-10">
              <div>
                <div className="relative w-full aspect-video">
                  <div className="w-full h-full rounded-lg bg-cover bg-center video-thumbnail bg-no-repeat" style={{ backgroundImage: `url(${videoData.thumbnail})` }}></div>
                  <span className="absolute p-1 opacity-70 bottom-2 right-2 bg-black text-white text-xs rounded">{videoData.duration_string}</span>
                </div>
              </div>
              <div>
                <strong>{videoData.title}</strong>
                <p>{videoData.channel ?? videoData.uploader}</p>
              </div>
            </div>
          )}
          <button id="run-downloader-button" className="btn btn-primary" onClick={runDownloader}>Descargar</button>
        </>
      }
    </div>
  )
}
