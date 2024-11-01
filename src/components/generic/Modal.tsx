import type { JSX } from 'solid-js';

interface Props {
  isOpen?: boolean;
  title?: string;
  message: string;
  isDismissable?: boolean;
  onClose?: () => void;
  children?: JSX.Element;
}

export default function Modal({ isOpen = false, title = "Mensaje", message, isDismissable = true, onClose, children }: Props) {
  return (
    <dialog class={`modal overflow-hidden backdrop-blur-sm backdrop-brightness-90 ${isOpen ? "modal-open" : ""}`}>
      <div class="modal-box text-left">
        {title && <h3 class="font-bold text-lg">{title}</h3>}
        <p class="pt-4">{message}</p>
        {children}
        {isDismissable &&
          <div class="modal-action">
            <form method="dialog">
              <button class="btn" onClick={() => onClose?.()}>Cerrar</button>
            </form>
          </div>
        }
      </div>
    </dialog>
  );
}