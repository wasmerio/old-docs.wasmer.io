extern int get_counter();
extern int add_to_counter(int value_to_add);

int increment_counter_loop(int number_of_times) {

  int current_counter = get_counter();
  
  for(int i = 0; i < number_of_times; i++) {
    current_counter = add_to_counter(1);
  }

  return current_counter;
}
