import { FC, useState } from "react"
import * as styles from "./MathInput.css"

const MathInput: FC = () => {
  const [equation, setEquation] = useState("")

  const updateEquation = (event: React.ChangeEvent<HTMLInputElement>) => {
    setEquation(event.target.value.toLowerCase())
  }

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      event.preventDefault()
      console.log(equation)
      setEquation("")
    }
  }

  return (
    <input
      className={styles.input}
      type="text"
      value={equation}
      onChange={updateEquation}
      onKeyDown={handleKeyDown}
    />
  )
}

export default MathInput