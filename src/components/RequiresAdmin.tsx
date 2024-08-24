import { invoke } from "@tauri-apps/api";
import Warning from "@/components/generic/Warning";

interface Props {
  text: string;
}

let isAdmin: boolean;

(async () => {
  isAdmin = await invoke('check_admin', { ask: false });
})();

export default function RequiresAdmin(props: Props) {
  return (
    isAdmin ? null : (
      <Warning text={props.text} buttonText="Reiniciar como administrador" onButtonClick={async () => await invoke('check_admin', { ask: true })} />
    )
  );
}