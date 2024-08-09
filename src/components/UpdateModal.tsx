import { useState, useEffect } from 'preact/hooks';
import { checkUpdate, installUpdate } from '@tauri-apps/api/updater';
import Modal from './Modal'
import Countdown from './Countdown'

export default function UpdateModal() {
  const [isOpen, setIsOpen] = useState(false)

  useEffect(() => {
    (async () => {
      const self = await checkUpdate();
      if (self.shouldUpdate) setIsOpen(true)
    })()
  })

  return (
    <Modal isOpen={isOpen} title="Actualización disponible" message="La aplicación se va a reiniciar para aplicar la actualización." isDismissable={false}>
      <Countdown initialNumber={3} onFinish={async () => await installUpdate()} />
    </Modal>
  )
}