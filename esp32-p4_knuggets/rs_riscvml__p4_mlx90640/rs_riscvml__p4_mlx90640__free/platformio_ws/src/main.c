#include <stdio.h>
#include <string.h>
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include "driver/i2c_master.h"
#include "esp_log.h"
#include "MLX90640_I2C_Driver.h"
#include "MLX90640_API.h"

static const char *TAG = "mlx90640";

#ifndef MLX90640_I2C_ADDR
#define MLX90640_I2C_ADDR 0x33
#endif
#ifndef I2C_SDA_GPIO
#define I2C_SDA_GPIO 22
#endif
#ifndef I2C_SCL_GPIO
#define I2C_SCL_GPIO 23
#endif
#ifndef I2C_FREQ_HZ
#define I2C_FREQ_HZ 400000
#endif

#define ROWS 24
#define COLS 32
#define PIXEL_COUNT (ROWS * COLS)

static uint16_t mlx_frame[834];
static float mlx_temperatures[PIXEL_COUNT];
static paramsMLX90640 mlx_params;

void app_main(void)
{
    ESP_LOGI(TAG, "MLX90640 Thermal Camera — RISCVML");
    ESP_LOGI(TAG, "I2C SDA=GPIO%d SCL=GPIO%d addr=0x%02X freq=%dHz",
             I2C_SDA_GPIO, I2C_SCL_GPIO, MLX90640_I2C_ADDR, I2C_FREQ_HZ);

    // Read EEPROM and extract calibration parameters
    uint16_t eeprom[832];
    int status = MLX90640_DumpEE(MLX90640_I2C_ADDR, eeprom);
    if (status != 0) {
        ESP_LOGE(TAG, "Failed to read EEPROM: %d", status);
        return;
    }

    status = MLX90640_ExtractParameters(eeprom, &mlx_params);
    if (status != 0) {
        ESP_LOGE(TAG, "Failed to extract parameters: %d", status);
        return;
    }

    // Set refresh rate to 4 Hz (0x04)
    MLX90640_SetRefreshRate(MLX90640_I2C_ADDR, 0x04);

    // Set resolution to 18-bit (0x02)
    MLX90640_SetResolution(MLX90640_I2C_ADDR, 0x02);

    ESP_LOGI(TAG, "Sensor initialized. Streaming frames...");

    float emissivity = 0.95f;
    float tr = 23.15f; // reflected temperature estimate

    while (1) {
        // TODO: Read both subpages from the MLX90640
        // TODO: Calculate temperatures using MLX90640_CalculateTo()
        // TODO: Output JSON frame over serial in format:
        //   {"rows":24,"cols":32,"vdd":3.3,"ta":25.1,"pixels":[t0,t1,...,t767]}
        vTaskDelay(pdMS_TO_TICKS(5000));
    }
}
