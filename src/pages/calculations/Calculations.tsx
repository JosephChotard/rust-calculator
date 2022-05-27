import { ColorModeToggle } from '../../components/colour-mode/ColourMode'
import MathInput from '../../components/math-input'
import { Box } from '../../components/system/box/Box'

function CalculationsPage() {

  return (
    <Box
      component='main'
      display='flex'
      flexDirection='column'
    >
      <Box
        display="flex"
        flexGrow={0}
        justifyContent="flex-end"
        paddingBottom={{
          mobile: 'large',
          tablet: 'none',
          desktop: 'xxlarge',
        }}
      >
        <ColorModeToggle />
      </Box>
      <Box
        flexGrow={1}
        display="flex"
        flexDirection="column"
        justifyContent="flex-end"
      >
        <MathInput />
      </Box>
    </Box>
  )
}

export default CalculationsPage
