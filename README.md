# ARM-Toolkit #

This is a set of build tools and source code designed for low-power ARM
processors (e.g. Cortex-M0). I've slowly put this together as I wrote code for
the LPC11Uxx, so you'll notice it's biased toward that family. There is no
reason it cannot be adapted for other processors, I just haven't gotten around
to it. Patches welcome!

## How the heck do I use this? ##

The toolkit is meant to sit adjacent to your code (perhaps as a submodule of
your larger repo) and be included into the Makefile. Your Makefile will need to
define a few variables **before** including the toolkit makefile.mk. You can
optionally define other variables to affect the output and logging. Please
refer to [makefile.mk](makefile.mk) for a full list of variables.

### Example ###

#### Makefile ####

```make
OUTPUT       := example
TOOLKIT_PATH := arm-toolkit

SOURCES := main.c

include $(TOOLKIT_PATH)/makefile.mk
```

#### main.c ####

```c
#include "cmsis.h"

int main(void)
{
	while (1)
		__WFI();
}

```
