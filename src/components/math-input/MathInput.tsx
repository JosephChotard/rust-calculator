import { tauri } from "@tauri-apps/api"
import { FC, useState } from "react"
import { Operation } from "../operation-history"
import { Box } from "../system/box/Box"
import { Text } from "../typography"
import * as styles from "./MathInput.css"


const MathInput: FC = () => {
  const [equation, setEquation] = useState("")
  const [response, setResponse] = useState("")

  const updateEquation = (event: React.ChangeEvent<HTMLInputElement>) => {
    let input = event.target.value.toLowerCase()
    if (["+", "-", "*", "/"].includes(input)) {
      input = "ans" + input
    }
    setEquation(input)

    if (input.length > 0) {
      tauri.invoke<number>("get_result_command", {
        input: input
      })
        .then((result) => {
          setResponse(result?.toString() ?? "Infinity")
        })
        .catch((error) => {
          if (error === "command") {
            setResponse("")
          } else {
            setResponse(error?.toString() ?? "")
          }
        })
    } else {
      setResponse("")
    }
  }

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      event.preventDefault()
      tauri.invoke<Operation>('store_operation_command', {
        input: equation,
      }).then(() => {
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
        spellCheck="false"
        autoCapitalize="off"
        autoComplete="off"
        autoCorrect="off"
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