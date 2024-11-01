import { createSignal, onMount } from 'solid-js';
import { invoke } from "@tauri-apps/api/core";
import Warning from "@components/generic/Warning";

interface Props {
  text: string;
}

export default function RequiresAdmin({ text }: Props) {
  const [isAdmin, setIsAdmin] = createSignal(false);

  onMount(() => {
    invoke("check_admin", { ask: false }).then((r) => {
      setIsAdmin(r as boolean);
    });
  });

  return (
    <>
      {isAdmin() && (
        <Warning
          text={text}
          buttonText="Reiniciar como administrador"
          onButtonClick={async () => await invoke("check_admin", { ask: true })}
        />
      )}
    </>
  );
}
