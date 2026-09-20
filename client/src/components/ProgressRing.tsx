export default function ProgressRing({
  done,
  total,
  size = 18,
}: {
  done: number
  total: number
  size?: number
}) {
  const radius = (size - 3) / 2
  const circumference = 2 * Math.PI * radius
  const fraction = total === 0 ? 0 : done / total
  const center = size / 2
  return (
    <svg
      width={size}
      height={size}
      viewBox={`0 0 ${size} ${size}`}
      aria-label={`${done} of ${total} done`}
      className="shrink-0"
    >
      <circle
        cx={center}
        cy={center}
        r={radius}
        fill="none"
        stroke="var(--color-edge)"
        strokeWidth={3}
      />
      <circle
        cx={center}
        cy={center}
        r={radius}
        fill="none"
        stroke={fraction === 1 ? 'var(--color-ok)' : 'var(--color-rust)'}
        strokeWidth={3}
        strokeDasharray={circumference}
        strokeDashoffset={circumference * (1 - fraction)}
        strokeLinecap="round"
        transform={`rotate(-90 ${center} ${center})`}
      />
    </svg>
  )
}
