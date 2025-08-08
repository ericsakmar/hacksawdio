function PlayIcon({ className }: { className?: string }) {
  return (
    <svg
      xmlns="http://www.w3.org/2000/svg"
      width="24"
      height="24"
      viewBox="0 0 24 24"
      fill="none"
      stroke="currentColor"
      strokeWidth="2"
      strokeLinecap="round"
      strokeLinejoin="round"
      className={className}
    >
      <polygon points="5 3 19 12 5 21 5 3"></polygon>
    </svg>
  );
  // <svg
  //   height="21"
  //   viewBox="0 0 21 21"
  //   width="21"
  //   xmlns="http://www.w3.org/2000/svg"
  // >
  //   <g
  //     fill="none"
  //     fillRule="evenodd"
  //     stroke="currentColor"
  //     strokeLinecap="round"
  //     strokeLinejoin="round"
  //     transform="translate(6 5)"
  //   >
  //     <path d="m.5.5v10l8-5z" />
  //   </g>
  // </svg>
  // );
}

export default PlayIcon;
