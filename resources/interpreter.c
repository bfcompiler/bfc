/**
 * Copyright (c) 2023 Jacob Allen Morris
 * 
 * This software is released under the MIT License.
 * https://opensource.org/licenses/MIT
 */

#include <stdio.h>

#define CELL_COUNT 30000
#define LOOP_COUNT 512
#define DATA_SIZE 0
#define RAW_DATA {0}
#define IS_WINDOWS 1

#if IS_WINDOWS == 1
    #include <conio.h>
#else
    #include <ncurses.h>
#endif

static int DATA[] = RAW_DATA;

int main() {
    #if IS_WINDOWS == 0
    initscr();
    cbreak();
    noecho();
    #endif
    int CELLS[CELL_COUNT] = {0};
    int POINTER = 0;
    int i = 0;
    int LOOPS[LOOP_COUNT] = {0};
    int LOOP_INDEX = 0;
    int complete = 0;
    while (complete < 1) {
        if (i == DATA_SIZE) {
            complete = 1;
        }
        if (DATA[i] == 0) {
            POINTER += 1;
            i += 1;
        } else if (DATA[i] == 1) {
            POINTER -= 1;
            i += 1;
        } else if (DATA[i] == 2) {
            if (CELLS[POINTER] == 255) {
                CELLS[POINTER] = 0;
            } else {
                CELLS[POINTER] += 1;
            }
            i += 1;
        } else if (DATA[i] == 3) {
            if (CELLS[POINTER] == 0) {
                CELLS[POINTER] = 255;
            } else {
                CELLS[POINTER] -= 1;
            }
            i += 1;
        } else if (DATA[i] == 4) {
            LOOPS[LOOP_INDEX] = i;
            LOOP_INDEX += 1;
            i += 1;
        } else if (DATA[i] == 5) {
            if (CELLS[POINTER] == 0) {
                LOOP_INDEX -= 1;
                i += 1;
            } else {
                i = LOOPS[LOOP_INDEX - 1];
            }
        } else if (DATA[i] == 6) {
            #if IS_WINDOWS == 1
            printf("%c", CELLS[POINTER]);
            #else
            printw("%c", CELLS[POINTER]);
            refresh();
            #endif
            i += 1;
        } else if (DATA[i] == 7) {
            int character = getch();
            #if IS_WINDOWS == 0
            refresh();
            #endif
            CELLS[POINTER] = character;
            i += 1;
        }
    }
    #if IS_WINDOWS == 1
    printf("\r\n");
    #else
    printw("\n[Execution Complete, press any key to continue]");
    refresh();
    getch();
    endwin();
    #endif
    return 0;
}