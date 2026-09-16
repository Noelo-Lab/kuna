// Function: dispatch @ 0x100000
void dispatch(unsigned int a0)
{
  char *v1; // stack - 0x4
  
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
}

// Function: emit @ 0x10003a
char emit(char *a0)
{
  return *a0;
}

