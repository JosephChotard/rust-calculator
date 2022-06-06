import { createRef, FC, useContext, useEffect } from "react"
import { Box } from "../system/box/Box"
import { Text } from "../typography"
import { OperationHistoryContext } from "./"
import * as styles from "./OperationHistory.css"


const OperationHistory: FC = () => {
  const { history } = useContext(OperationHistoryContext)
  const elementRef = createRef<HTMLDivElement>()

  useEffect(() => {
    elementRef.current?.scrollIntoView()
  }, [history])

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
          <Text size='small'>
            {"➛"} <Box component="span" className={styles.result}>{operation.result ?? Infinity}</Box>
          </Text>
        </Box>
      ))}
      <div ref={elementRef} />
    </Box>
  )
}

export default OperationHistory