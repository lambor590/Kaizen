import { installUpdate } from '@tauri-apps/api/updater';
import Modal from '@/components/generic/Modal'
import Countdown from '@/components/generic/Countdown'

export default function UpdateModal() {

  return (
    <Modal isOpen={true} title="Actualización disponible" message="La aplicación se va a reiniciar para aplicar la actualización." isDismissable={false}>
      <Countdown initialNumber={3} onFinish={async () => await installUpdate()} />
    </Modal>
  )
}