# Project Description

Open Source software to enchance and make game modding easier *hopefully*


## An Overview of Application Structure

`app_backend` is for the backend portion of the application

`app_gui` is the graphical interface of the application


The program is seperated into two portions so that the graphical interface portion of the application can be modified without the worry of harming the actual functionality of the program, this also allows the backend to function more as an API and the graphical interface is just a layer on top that can be interchanged. Doing it this way will minimize the destructiveness of my tendency to rewrite unsavory or messy code, which is what my GUI code usually looks like most of the time anyways.