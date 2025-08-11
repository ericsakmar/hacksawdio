function LoaderIcon({ className }: { className?: string }) {
  return (
    <svg
      height="21"
      viewBox="0 0 21 21"
      width="21"
      xmlns="http://www.w3.org/2000/svg"
      className={className}
    >
      <g
        fill="none"
        fillRule="evenodd"
        stroke="currentColor"
        strokeLinecap="round"
        strokeLinejoin="round"
      >
        <path d="m10.5 3.5v2" />
        <path d="m15.5 5.5-1.5 1.5" />
        <path d="m5.5 5.5 1.5 1.5" />
        <path d="m10.5 17.5v-2" />
        <path d="m15.5 15.5-1.5-1.5" />
        <path d="m5.5 15.5 1.5-1.5" />
        <path d="m3.5 10.5h2" />
        <path d="m15.5 10.5h2" />
      </g>
    </svg>
  );
}

export default LoaderIcon;
