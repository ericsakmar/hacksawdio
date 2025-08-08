interface Props {
  animated?: boolean;
}

function Logo({ animated = false }: Props) {
  const classes = [
    "text-2xl",
    "text-center",
    "bg-gradient-to-r",
    "from-green-400",
    "to-blue-500",
    "bg-clip-text",
    "text-transparent",
    "font-accent",
    // "uppercase",
  ];

  if (animated) {
    classes.push("animate-pulse");
  }

  return <h1 className={classes.join(" ")}>hacksawdio</h1>;
}

export default Logo;
