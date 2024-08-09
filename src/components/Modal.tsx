import { ComponentChildren } from "preact";

interface Props {
  isOpen: boolean;
  title?: string;
  message: string;
  children?: ComponentChildren;
  isDismissable?: boolean;
  onClose?: () => void;
}

export default function Modal({ isOpen, title = "", message, children, isDismissable = true, onClose }: Props) {
  return (
    <dialog className={`modal overflow-hidden backdrop-blur-sm backdrop-brightness-90 ${isOpen ? 'modal-open' : ''}`}>
      <div className="modal-box text-left">
        <h3 className="font-bold text-lg">{title}</h3>
        <p className="pt-4">{message}</p>
        {children}
        {
          isDismissable &&
          <div className="modal-action">
            <form method="dialog">
              <button className="btn" onClick={onClose}>Cerrar</button>
            </form>
          </div>
        }
      </div>
    </dialog>
  );
}