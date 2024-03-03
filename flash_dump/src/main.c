/*******************************************************************************
* Copyright (C) 2021 Maxim Integrated Products, Inc., All Rights Reserved.
*
* Permission is hereby granted, free of charge, to any person obtaining a
* copy of this software and associated documentation files (the "Software"),
* to deal in the Software without restriction, including without limitation
* the rights to use, copy, modify, merge, publish, distribute, sublicense,
* and/or sell copies of the Software, and to permit persons to whom the
* Software is furnished to do so, subject to the following conditions:
*
* The above copyright notice and this permission notice shall be included
* in all copies or substantial portions of the Software.
*
* THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
* OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
* MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
* IN NO EVENT SHALL MAXIM INTEGRATED BE LIABLE FOR ANY CLAIM, DAMAGES
* OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
* ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
* OTHER DEALINGS IN THE SOFTWARE.
*
* Except as contained in this notice, the name of Maxim Integrated
* Products, Inc. shall not be used except as stated in the Maxim Integrated
* Products, Inc. Branding Policy.
*
* The mere transfer of this software does not imply any licenses
* of trade secrets, proprietary technology, copyrights, patents,
* trademarks, maskwork rights, or any other form of intellectual
* property whatsoever. Maxim Integrated Products, Inc. retains all
* ownership rights.
*******************************************************************************/

/***** Includes *****/
#include <stdio.h>
#include <string.h>
#include <stdint.h>
#include "mxc_delay.h"
#include "mxc_sys.h"
#include "flc.h"
#include "led.h"

#define TEST_ADDRESS (MXC_FLASH_MEM_BASE + MXC_FLASH_MEM_SIZE) - (1 * MXC_FLASH_PAGE_SIZE)
/***** Globals *****/
uint32_t START_ADDRESS = MXC_FLASH_MEM_BASE;//32buf x 64cnt x 4byte=8192=0x2000=MXC_FLASH_PAGE_SIZE
/***** Functions *****/
//0x10080000UL


int main(void){
    while (START_ADDRESS < MXC_FLASH_MEM_BASE+MXC_FLASH_MEM_SIZE) { //< MXC_FLASH_MEM_SIZE
        LED_On(LED1);
        MXC_Delay(MXC_DELAY_MSEC(500));
        LED_Off(LED1);
        MXC_Delay(MXC_DELAY_MSEC(500)); 
        //uint32_t buffer[32];
        unsigned char one_byte;
        int i;
        //for(i = 0; i < 128 && START_ADDRESS < (MXC_FLASH_MEM_BASE+MXC_FLASH_MEM_SIZE); ++i){
        MXC_FLC_Read(START_ADDRESS, &one_byte, 1);
        //fprintf(file, "%d ", one_4byte);
        printf("%c", one_byte);
        START_ADDRESS++;
        //MXC_Delay(MXC_DELAY_MSEC(1000));
        //}
        //fprintf(file, "\n");
        //printf("\n");

        // uint32_t one_4byte = 0;
        // MXC_FLC_Read(START_ADDRESS, &one_4byte, 4);
        // fprintf(file, );
    }
    LED_Off(LED1);

    //close(file);
    return 0;
}
