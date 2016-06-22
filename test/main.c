#if defined(LPC11U24_301)
# include "cmsis.h"
#elif defined(EFM32LG990F256)
# include "efm32lg990f256.h"
#endif

int main(void)
{
	while (1)
		__WFI();
}

