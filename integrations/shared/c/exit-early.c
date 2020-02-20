extern void interrupt_execution();
extern void should_not_be_called();

int exit_early() {
  
  // Interrupt the execution of this function
  // By calling the intended function from the host application
  interrupt_execution();

  // This will never called, and never get returned
  should_not_be_called();
  return 24;
}
