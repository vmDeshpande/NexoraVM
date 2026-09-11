import type { ButtonHTMLAttributes, ReactNode } from "react";

type ButtonVariant = "primary" | "secondary" | "ghost";

interface ButtonProps extends ButtonHTMLAttributes<HTMLButtonElement> {
  children: ReactNode;
  variant?: ButtonVariant;
}

export function Button({
  children,
  className = "",
  variant = "secondary",
  ...props
}: ButtonProps) {
  const classes = ["button", `button--${variant}`, className].filter(Boolean);

  return (
    <button type="button" className={classes.join(" ")} {...props}>
      {children}
    </button>
  );
}
