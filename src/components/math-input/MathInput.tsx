import { tauri } from "@tauri-apps/api"
import { FC, useContext, useState } from "react"
import { Operation, OperationHistoryContext } from "../operation-history"
import * as styles from "./MathInput.css"

const MathInput: FC = () => {
  const [equation, setEquation] = useState("")
  const { addToHistory } = useContext(OperationHistoryContext)

  const updateEquation = (event: React.ChangeEvent<HTMLInputElement>) => {
    setEquation(event.target.value.toLowerCase())
  }

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      event.preventDefault()
      console.log(equation)
      tauri.invoke<Operation>('store_operation_command', {
        input: equation,
      }).then((operation) => {
        addToHistory(operation)
      }).finally(() => {
        setEquation("")
      })
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