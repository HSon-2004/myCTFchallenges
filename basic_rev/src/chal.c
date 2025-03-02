#include <stdio.h>
#include <string.h>

int main() {
    char input[256];
    printf("You must enter a password to receive the flag: ");
    fgets(input, sizeof(input), stdin);
    input[strcspn(input, "\n")] = '\0';  

    int input_enc[256] = {0};
    for (int i = 0; i < strlen(input); i++) {
        input_enc[i] = input[i] ^ 0x55;
    }


    const int expected_enc[] = {0x0c, 0x30, 0x34, 0x3d, 0x0a, 0x21, 0x3d, 0x3c, 0x26, 0x0a, 0x3c, 0x26, 0x0a, 0x34, 0x0a, 0x3e, 0x30, 0x2c, 0x74};
    size_t expected_enc_len = sizeof(expected_enc) / sizeof(expected_enc[0]);

    int flag_enc[] = {27, 46, 40, 59, 28, 15, 49, 90, 0, 126, 54, 42, 111, 20, 0, 12, 85, 13, 126, 45, 13, 82, 55, 57, 24, 92, 14, 82, 34};


    if (memcmp(input_enc, expected_enc, expected_enc_len * sizeof(int)) == 0) {
        printf("Flag is: ");
        for (int i = 0; i < sizeof(flag_enc) / sizeof(flag_enc[0]); i++) {
            char flag_char = flag_enc[i] ^ input[i % strlen(input)];
            printf("%c", flag_char);
        }
        printf("\n");
    } else {
        printf("Wrong password.\n");
    }

    return 0;
}
