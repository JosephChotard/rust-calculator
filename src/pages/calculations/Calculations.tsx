import { ColorModeToggle } from '../../components/colour-mode/ColourMode'
import MathInput from '../../components/math-input'
import { OperationHistory, OperationHistoryProvider } from '../../components/operation-history'
import { Box } from '../../components/system/box/Box'

function CalculationsPage() {

  return (
    <Box
      component='main'
      display='flex'
      flexDirection='column'
    >
      <Box
        paddingBottom={{
          mobile: 'large',
          tablet: 'none',
          desktop: 'xxlarge',
        }}
        position="fixed"
        top="medium"
        right="medium"
      >
        <ColorModeToggle />
      </Box>
      <OperationHistoryProvider>
        <Box
          display="flex"
          flexDirection="column"
          justifyContent="flex-end"
          height="full"
        >
          <OperationHistory />
          <Box
            position='fixed'
            bottom='none'
            width='full'
          >
            <MathInput />
          </Box>
        </Box>
      </OperationHistoryProvider>
    </Box>
  )
}

export default CalculationsPage
