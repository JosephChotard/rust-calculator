import { tauri } from "@tauri-apps/api"
import { FC, useContext, useEffect, useState } from "react"
import { Operation } from "../operation-history"
import { Box } from "../system/box/Box"
import { Text } from "../typography"
import { CurrentOperationContext } from "./CurrentOperationContext"
import * as styles from "./MathInput.css"


const MathInput: FC = () => {
  const { operation, setOperation } = useContext(CurrentOperationContext)
  const [response, setResponse] = useState("")

  const handleOnChange = (event: React.ChangeEvent<HTMLInputElement>) => {
    let input = event.target.value.toLowerCase()

    /* Allow the user to start a new equation without having to type in `ans` first. */
    if (["+", "-", "*", "/"].includes(input)) {
      input = "ans" + input
    }
    updateEquation(input)
    setOperation(input)
  }

  const updateEquation = (input: string) => {

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

  useEffect(() => {
    updateEquation(operation)
  }, [operation])

  const handleKeyDown = (event: React.KeyboardEvent<HTMLInputElement>) => {
    if (event.key === "Enter") {
      event.preventDefault()
      tauri.invoke<Operation>('store_operation_command', {
        input: operation,
      }).then(() => {
        setOperation("")
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
        value={operation}
        onChange={handleOnChange}
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