# boot.S
# bootloader for SoS
# All credits to Stephen Marz

.option norvc
.section .data

.section .text.init
.global _start

_start:
