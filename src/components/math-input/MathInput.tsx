import { tauri } from "@tauri-apps/api"
import { FC, useContext, useState } from "react"
import { Operation, OperationHistoryContext } from "../operation-history"
import { Box } from "../system/box/Box"
import { Text } from "../typography"
import * as styles from "./MathInput.css"


const MathInput: FC = () => {
  const [equation, setEquation] = useState("")
  const [response, setResponse] = useState("")
  const { addToHistory } = useContext(OperationHistoryContext)

  const updateEquation = (event: React.ChangeEvent<HTMLInputElement>) => {
    const input = event.target.value.toLowerCase()
    setEquation(input)
    if (input.length > 0) {
      tauri.invoke<number>("get_result_command", {
        input: input
      })
        .then((result) => {
          setResponse(result?.toString() ?? "Infinity")
        })
        .catch((error) => {
          setResponse(error?.toString() ?? "")
        })
    } else {
      setResponse("")
    }
  }

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      event.preventDefault()
      console.log(equation)
      tauri.invoke<Operation>('store_operation_command', {
        input: equation,
      }).then((operation) => {
        addToHistory(operation)
        setEquation("")
        setResponse("")
      })
    }
  }

  return (
    <Box
      className={styles.inputWrapper}
    >
      <input
        className={styles.input}
        type="text"
        value={equation}
        onChange={updateEquation}
        onKeyDown={handleKeyDown}
      />
      {response && (
        <Text
          className={styles.response}
          size="small"
          invertedColour={true}
        >
          {response}
        </Text>
      )}
    </Box>
  )
}

export default MathInput