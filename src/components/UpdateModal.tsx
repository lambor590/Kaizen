import { createSignal, onMount } from 'solid-js';
import { check } from '@tauri-apps/plugin-updater';
import { relaunch } from '@tauri-apps/plugin-process';
import Modal from '@components/generic/Modal';
import Countdown from '@components/generic/Countdown';

export default function UpdateModal() {
  const [updateAvailable, setUpdateAvailable] = createSignal(false);
  const title = "Actualización disponible";
  const message = "La aplicación se va a reiniciar para actualizarse.";

  onMount(async () => {
    const update = await check();
    if (update) {
      setUpdateAvailable(true);
      update.downloadAndInstall();
    }
  });

  const handleFinish = async () => {
    await relaunch();
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
