import { createSignal, onCleanup } from 'solid-js';

interface Props {
  initialNumber?: number;
  onFinish?: () => void;
}

export default function Countdown({ initialNumber = 0, onFinish }: Props) {
  const [timeLeft, setTimeLeft] = createSignal(initialNumber);

  const timer = setInterval(() => {
    if (timeLeft() === 0) {
      clearInterval(timer);
      onFinish?.();
      return;
    }
    setTimeLeft(timeLeft() - 1);
  }, 1000);

  onCleanup(() => clearInterval(timer));

  return (
    <div class="countdown font-mono text-6xl flex flex-col items-center">
      <span style={{ "--value": timeLeft() }}></span>
    </div>
  );
}
