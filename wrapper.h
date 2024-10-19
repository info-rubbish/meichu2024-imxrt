#include <imxrt_lpi2c.h>
#include <imxrt_gpio.h>
#include <malloc.h>
#include <stdio.h>
#include <nuttx/board.h>
#include <nuttx/input/touchscreen.h>
#include <nuttx/irq.h>
#include <nuttx/lcd/lcd_dev.h>
#include <nuttx/nuttx.h>
#include <nuttx/video/fb.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/ioctl.h>
#include <sys/socket.h>

// driver
#include <nuttx/input/gt9xx.h>
