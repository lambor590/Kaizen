import { useEffect, useState } from 'preact/hooks';

interface Props {
  initialNumber: number;
  onFinish?: () => void;
}

export default function Countdown(props: Props) {
  const [timeLeft, setTimeLeft] = useState(props.initialNumber);

  useEffect(() => {
    let timer = setInterval(() => {
      setTimeLeft((time) => {
        if (time !== 0) return time - 1;
        clearInterval(timer);
        props.onFinish?.();
        return 0;
      });
    }, 1000);
  }, []);

  return (
    <div className="countdown font-mono text-6xl flex flex-col items-center">
      <span style={{ "--value": timeLeft }}></span>
    </div>
  )
}