# How Do The Motors Work?
Before we start programming the motors its worth explaining how the micro:bit is able to communicate with the motors

## Acceleration Motors

An acceleration motor is a motor whos only job is to turn 360 degrees forwards or backwards and do nothing else. You cannot explicitly tell this motor "hey, I want to go to 180 degrees" (Its not impossible to make it stop at 180 degrees exactly though. you just have to code that yourself)

Typically when you are interacting with this kind of motor in the real world its logic is simple. The more voltage you pour into it, the faster it goes. 

Theres a problem with this when we're programming with the micro:bit though. The microbit has no way to change the amount voltage it puts out. this must mean its impossible to change how fast a motor goes then. Right?


## PWM (Pulse Width Modulation)
To communicate with components that require a variable amount of voltage we can use Pulse Width Modulation or PWM. PWM switches our power source between on and off at a certain interval to simulate a steady lower voltage. This can let us supply 2.5v of power with a 5v power source. 

![Visual Representation of PWM](https://cdn.sparkfun.com/assets/f/9/c/8/a/512e869bce395fbc64000002.JPG)

(Credit: [SparkFun Electronics](https://learn.sparkfun.com/tutorials/pulse-width-modulation/all))


## Turning Motors

A turning motor (Or a Servo Motor) is a motor where instead of speeding up when you give it more voltage. It moves from one angle to another. Basically unlike Acceleration motors you can tell it "Hey I want to go 180 degrees" and it will actually do that.



## The Chip in charge of the motors

Technically the Micro:bit could control the motors by itself (with the help of some external power source) but that would take up processing power on the microcontroller so the duty of managing all of our PWM outputs is shoved onto an external chip: The PCA9685. All this chip does is control our PWM outputs without the micro:bit having to manage them all.
