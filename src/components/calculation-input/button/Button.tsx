import React, { FC } from "react"
import { buttonStyle } from "./button.css"

interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  theme: keyof typeof buttonStyle
}
const Button: FC<ButtonProps> = ({
  children,
  theme,
  ...props
}) => {
  return (
    <button {...props} className={buttonStyle[theme]}>
      {children}
    </button>
  )
}

export default Button