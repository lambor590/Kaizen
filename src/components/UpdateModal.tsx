import { createSignal, onMount } from 'solid-js';
import { check, Update } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import Modal from '@components/generic/Modal';
import Countdown from '@components/generic/Countdown';

export default function UpdateModal() {
  const [updateAvailable, setUpdateAvailable] = createSignal(false);
  const title = "Actualización disponible";
  const message = "La aplicación se va a reiniciar para actualizarse.";

  let update: Update | null = null;
  onMount(async () => {
    update = await check();
    if (update) {
      setUpdateAvailable(true);
      await update.download();
    }
  });

  const handleFinish = async () => {
    await update?.install();
  };

  return (
    <>
      {updateAvailable() && (
        <Modal isOpen={true} title={title} message={message} isDismissable={false}>
          <Countdown initialNumber={3} onFinish={handleFinish} />
        </Modal>
      )}
    </>
  );
}
