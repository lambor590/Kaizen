import CheckBox from "@/components/generic/Checkbox";

export default function Roadmap() {
  return (
    <div className="prose">
      <h1>Lista de tareas para la 1.0</h1>
      <p className="text-warning">El orden no importa.</p>
      <div>
        <CheckBox text="Interfaz y diseño" checked disabled />
        <CheckBox text="Sistema de actualizaciones" checked disabled />
        <CheckBox text="Liberador de espacio" checked disabled />
        <CheckBox text="Apartado de configuración" disabled />
        <CheckBox text="Guardado de preferencias" disabled />
        <CheckBox text="Sistema de cuentas" disabled />
        <CheckBox text="Guardado en la nube" disabled />
        <CheckBox text="Descarga de vídeos/audios de muchas plataformas" disabled />
        <CheckBox text="Activador de Windows permanente" disabled />
        <CheckBox text="Estadísticas de uso públicas" disabled />
      </div>
    </div>
  )
}