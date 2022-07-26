import { createRef, FC, useContext, useEffect } from "react"
import { CurrentOperationContext } from "../math-input"
import { Box } from "../system/box/Box"
import { Text } from "../typography"
import { OperationHistoryContext } from "./"
import * as styles from "./OperationHistory.css"


const OperationHistory: FC = () => {
  const { history } = useContext(OperationHistoryContext)
  const { setOperation } = useContext(CurrentOperationContext)
  const elementRef = createRef<HTMLDivElement>()

  useEffect(() => {
    elementRef.current?.scrollIntoView()
  }, [history])

  const handleResultClick = (event: React.MouseEvent<HTMLButtonElement>, result: number) => {
    /* If the user double clicked the result append the result to the current operation. */
    if (event.detail == 2) {
      event.preventDefault()
      event.stopPropagation()
      setOperation(operation => `${operation}${result}`)
    }
  }

  return (
    <Box
      component='ul'
      padding='small'
      className={styles.historyContainer}
    >
      {history.map((operation, index) => (
        <Box
          component='li'
          key={index}
          className={styles.historyItem}
        >
          <Text size='small' color={{
            lightMode: 'red900',
            darkMode: 'red50',
          }}>
            {operation.operation}
          </Text>
          <Text size='small' >
            {"➛"} <button className={styles.result} onClick={(e) => handleResultClick(e, operation.result)}>{operation.result ?? Infinity}
            </button>
          </Text>
        </Box>
      ))}
      <div ref={elementRef} />
    </Box>
  )
}

export default OperationHistory