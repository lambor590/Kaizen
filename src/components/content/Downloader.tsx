import { createSignal, onMount } from "solid-js";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import Label from "@components/generic/Label";

type VideoData = {
  title: string;
  thumbnail: string;
  duration_string: string;
  channel: string;
  uploader: string;
  original_url: string;
};

export default function DownloaderContent() {
  const [config, setConfig] = createSignal({
    video_url: "",
    format: "video",
    quality: "best",
    output_folder: "",
    pitch: "1.0",
  });

  const [videoData, setVideoData] = createSignal<VideoData | null>(null);
  const [depsInstalled, setDepsInstalled] = createSignal(true);

  onMount(() => {
    invoke<boolean>("check_downloader_deps").then(
      (installed) => setDepsInstalled(installed)
    );
  });

  const handleChange = async (e: Event) => {
    const { name, value } = e.target as HTMLInputElement;
    setConfig((prev) => ({ ...prev, [name]: value }));

    if (
      name === "video_url" &&
      (!videoData() || value !== videoData()?.original_url)
    ) {
      const dataJson = await invoke<string>("get_video_data", {
        url: value,
      });
      setVideoData(dataJson ? JSON.parse(dataJson) : null);
    }
  };

  const selectFolder = async () => {
    const selected = await open({
      directory: true,
      title: "Selecciona la carpeta donde se descargará el contenido",
    });
    if (selected && typeof selected === "string") {
      setConfig((prev) => ({ ...prev, output_folder: selected }));
      (document.getElementById("folder-path") as HTMLInputElement).value = selected;
    }
  };

  const performAction = async (
    action: () => Promise<void>,
    buttonId: string,
  ) => {
    const button = document.getElementById(buttonId) as HTMLButtonElement;
    button.classList.add("btn-disabled");
    await action();
    button.classList.remove("btn-disabled");
  };

  const installDeps = () =>
    performAction(async () => {
      await invoke("install_downloader_deps");
      setDepsInstalled(true);
    }, "install-deps-button");

  const runDownloader = () =>
    performAction(async () => {
      if (config().video_url && config().output_folder) {
        await invoke("run_downloader", { config: config() });
      }
    }, "run-downloader-button");

  return (
    <>
      {!depsInstalled() ? (
        <>
          <p>
            No tienes los programas necesarios para descargar contenido.<br />
            Presiona este botón para descargarlos automáticamente donde está instalado Kaizen.<br />
            Solo tomará unos segundos.
          </p>
          <button
            id="install-deps-button"
            class="btn btn-primary"
            onClick={installDeps}
          >
            Descargar
          </button>
        </>
      ) : (
        <>
          <div class="grid grid-cols-2 gap-8 mb-10">
            <Label text="URL">
              <input
                type="text"
                placeholder="https://youtu.be/dQw4w9WgXcQ"
                name="video_url"
                class="input input-bordered w-full"
                onBlur={handleChange}
              />
            </Label>
            <Label text="Carpeta de destino">
              <input
                type="text"
                onClick={selectFolder}
                id="folder-path"
                placeholder="Seleccionar carpeta de destino"
                class="input input-bordered w-full cursor-pointer"
                readOnly
              />
            </Label>
            <Label text="Formato">
              <select
                name="format"
                class="select select-bordered w-full"
                onChange={handleChange}
              >
                <option value="video">Solo vídeo (si está disponible)</option>
                <option value="audio">Solo audio</option>
                <option value="both">Vídeo y audio</option>
              </select>
            </Label>
            <Label text="Calidad">
              <select
                name="quality"
                class="select select-bordered w-full"
                onChange={handleChange}
              >
                <option value="best">Mejor</option>
              </select>
            </Label>
            <Label text="Tono">
              <select
                name="pitch"
                class="select select-bordered w-full"
                onChange={handleChange}
              >
                <option value="1.6">Muy agudo</option>
                <option value="1.4">Agudo</option>
                <option value="1.2">Un poco agudo</option>
                <option value="1.0" selected>Normal</option>
                <option value="0.8">Un poco grave</option>
                <option value="0.6">Grave</option>
                <option value="0.4">Muy grave</option>
              </select>
            </Label>
          </div>
          {videoData() && (
            <div class="grid grid-cols-2 gap-8 mb-10">
              <div>
                <div class="relative w-full aspect-video">
                  <div
                    class="w-full h-full rounded-lg bg-cover bg-center video-thumbnail bg-no-repeat"
                    style={{
                      "background-image": `url(${videoData()?.thumbnail})`,
                    }}
                  ></div>
                  <span class="absolute p-1 opacity-70 bottom-2 right-2 bg-black text-white text-xs rounded">
                    {videoData()?.duration_string}
                  </span>
                </div>
              </div>
              <div>
                <strong>{videoData()?.title}</strong>
                <p>{videoData()?.channel ?? videoData()?.uploader}</p>
              </div>
            </div>
          )}
          <button
            id="run-downloader-button"
            class="btn btn-primary"
            onClick={runDownloader}
          >
            Descargar
          </button>
        </>
      )}
    </>
  );
}
