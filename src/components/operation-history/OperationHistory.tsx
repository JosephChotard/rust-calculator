import { createRef, FC, useContext, useEffect } from "react"
import { Box } from "../system/box/Box"
import { Text } from "../typography"
import { OperationHistoryContext } from "./"
import { historyContainer } from "./OperationHistory.css"


const OperationHistory: FC = () => {
  const { history } = useContext(OperationHistoryContext)
  const elementRef = createRef<HTMLDivElement>()

  useEffect(() => {
    elementRef.current?.scrollIntoView()
  }, [history])

  return (
    <Box className={historyContainer}>
      <Box
        component='ul'
        padding='small'
      >
        {history.map((operation, index) => (
          <Box
            component='li'
            key={index}
          >
            <Text size='code'>
              {operation.operation} {"=>"} {operation.result}
            </Text>
          </Box>
        ))}
        <div ref={elementRef} />
      </Box>
    </Box>
  )
}

export default OperationHistory