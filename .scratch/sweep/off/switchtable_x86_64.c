// Function: dispatch @ 0x100000
unsigned long dispatch(unsigned int a0)
{
  char *v1; // rdi
  
  switch(a0) {
    case 0:
      v1 = "switch case alpha reached";
      break;
    case 1:
      v1 = "switch case beta reached";
      break;
    case 2:
      v1 = "switch case gamma reached";
      break;
    case 3:
      v1 = "switch case delta reached";
      break;
    default:
      v1 = "switch default reached";
    
  }
  emit(v1);
  return 0;
}

// Function: emit @ 0x100041
char emit(char *a0)
{
  return *a0;
}

