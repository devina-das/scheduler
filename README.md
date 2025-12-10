Group Name: Group 11
Members: Devina (djdas2) & Barry (barryl2)

Project Introduction:
Our project is a to-do list application that will let users create, view, and remove reminders for tasks.
This will provide users with an efficient way to manage their responsibilities and deadlines.

Goals & Objectives: 
- Allow users to add, edit, remove, and display their tasks.
- Learn and implement ways to store data between different user sessions.
- Emphasize user-friendliness (make sure it's not too difficult/confusing to use).
      
Technical Descriptions:
Users will interact with the program through commands such as add, edit, remove, and display. 
- Add: Will allow the user to add a task with the following information (Title, Date, Time, Description).
- Edit: Will allow the user to edit some part of an existing task.
- Remove: Will allow the user to remove a task.
- Display: Will display all the tasks clearly to the user.

We will read/write to a file to store lists between user sessions. We may look into other ways to do this.

Checkpoint 1: 
- Develop the Task struct and DayOfWeek enum
- Learn how to work with files in Rust

Checkpoint 2:
- Implement the 4 functions - Add, Edit, Remove, Display
- Begin refining for improved user experience

Possible challenges:
- Error Handling with files, and formatting (ex, time can be entered in different ways).
- Making all tasks appear unique (might include an ID system?).
- The user instructions are not clear enough for users.
- Learning how to work with files

TODO:
- Incorporate egui_datepicker to replace current datepicker
- Timestamps for timezone support?
- Support ics?
  
References: Barry mentioned it was an example project shared at the find-a-group meeting.

