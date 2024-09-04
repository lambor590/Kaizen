import { invoke } from "@tauri-apps/api";
import { useEffect, useState } from "preact/hooks";
import { open } from "@tauri-apps/api/dialog";

type VideoFormat = {
  format_id: string;
  format_note: string;
  filesize: number;
};

type VideoData = {
  title: string;
  thumbnail: string;
  duration_string: string;
  channel: string;
  uploader: string;
  formats: VideoFormat[];
  original_url: string;
};

export default function Downloader() {
  const [downloaderConfig, setDownloaderConfig] = useState({
    video_url: "",
    format: "video",
    quality: "best",
    output_folder: "",
  });

  const [videoData, setVideoData] = useState<VideoData>();
  const [depsInstalled, setDepsInstalled] = useState(true);

  const getVideoData = async (url: string) => {
    const dataJson = await invoke<string>("plugin:tools|get_video_data", { url });
    return dataJson ? JSON.parse(dataJson) : null;
  }

  const handleChange = (e: Event) => {
    const { name, value } = e.target as HTMLInputElement;

    if (name === "video_url" && (!videoData || value !== videoData.original_url)) {
      getVideoData(value).then(data => {
        setVideoData(data);
      });
    }

    setDownloaderConfig(prev => ({ ...prev, [name]: value }));
  };

  const selectFolder = async () => {
    try {
      const selected = await open({ directory: true, title: "Selecciona la carpeta donde se descargarán los vídeos/audios" });

      if (selected && typeof selected === "string") {
        setDownloaderConfig(prev => ({ ...prev, output_folder: selected }));
        (document.getElementById("folder-path") as HTMLInputElement).value = selected;
      } else {
        console.error("Resultado de selección de carpeta inesperado:", selected);
      }
    } catch (error) {
      console.error("Error al seleccionar carpeta:", error);
    }
  }

  useEffect(() => {
    invoke<boolean>("plugin:tools|check_downloader_deps").then(setDepsInstalled);
  }, []);

  const installDeps = async (button: HTMLButtonElement) => {
    button.classList.add("btn-disabled");
    await invoke("plugin:tools|install_downloader_deps");
    button.classList.remove("btn-disabled");
    setDepsInstalled(true);
  }

  const runDownloader = async (button: HTMLButtonElement) => {
    if (!downloaderConfig.video_url || !downloaderConfig.output_folder) return;

    button.classList.add("btn-disabled");
    await invoke("plugin:tools|run_downloader", { config: downloaderConfig })
    button.classList.remove("btn-disabled");
  };

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
          <button id="btn-deps" className="btn btn-primary" onClick={(e) => installDeps(e.target as HTMLButtonElement)}>Descargar</button>
        </>
        :
        <>
          <div className="grid grid-cols-2 gap-8 mb-10">
            <label className="form-control">
              <div className="label">
                <span className="label-text">URL</span>
              </div>
              <input type="text" placeholder="https://youtu.be/dQw4w9WgXcQ" name="video_url" className="input input-bordered w-full" onFocusOut={handleChange} />
            </label>
            <label className="form-control">
              <div className="label">
                <span className="label-text">Carpeta de destino</span>
              </div>
              <input type="text" onClick={selectFolder} id="folder-path" placeholder="Seleccionar carpeta de destino" className="input input-bordered w-full cursor-pointer" readOnly />
            </label>
            <label className="form-control">
              <div className="label">
                <span className="label-text">Formato</span>
              </div>
              <select name="format" className="select select-bordered w-full" onChange={handleChange}>
                <option value="video">Vídeo sin audio (si está disponible)</option>
                <option value="audio">Audio sin vídeo</option>
                <option value="both">Vídeo y audio</option>
              </select>
            </label>
            <label className="form-control">
              <div className="label">
                <span className="label-text">Calidad</span>
              </div>
              <select name="quality" className="select select-bordered w-full" onChange={handleChange}>
                <option value="best">Mejor</option>
              </select>
            </label>
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
          <button className="btn btn-primary" onClick={(e) => runDownloader(e.target as HTMLButtonElement)}>Descargar</button>
        </>
      }
    </div>
  )
}
