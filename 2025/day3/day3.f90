program day3
    implicit none
    character(len=100), dimension(:), allocatable :: banks
    integer :: io

    open(newunit=io, file="day3.txt", status="old", action="read")

    block
        integer :: ios, num_lines
        num_lines = 0
        do
            read(io, *, iostat=ios)
            if (ios/= 0) exit
            num_lines = num_lines + 1
        end do
        rewind(io)

        allocate(banks(num_lines))
    end block
    
    block
        integer :: i
        do i = 1, size(banks)
            read(io, *) banks(i)
        end do
    end block

    block
        integer :: part1_result
        real :: start_time, end_time
        call cpu_time(start_time)
        part1_result = part1(banks)
        call cpu_time(end_time)
        print *, part1_result, " Time taken: ", end_time - start_time
    end block

    block
        integer*8 :: part2_result
        real :: start_time, end_time
        call cpu_time(start_time)
        part2_result = part2(banks)
        call cpu_time(end_time)
        print *, part2_result, " Time taken: ", end_time - start_time
    end block

contains

    function part1(banks) result(count)
        implicit none
        character(len=100), dimension(:), intent(in) :: banks
        integer :: count, i, bank_len
    
        count = 0
        bank_len = len_trim(banks(1))

        do i = 1, size(banks)
            block
                character :: first_char, second_char
                integer :: index

                first_char = banks(i)(1:1)
                second_char = '0'
                
                ! From the second character onwards, either find a character that is large than our first character!
                ! and move the start of our sequence to that point
                ! Or find a character that is larger than our second character and just replace our second character.
                do index = 2, bank_len
                    if (index < bank_len .and. iachar(banks(i)(index:index)) > iachar(first_char)) then
                        first_char = banks(i)(index:index)
                        second_char = banks(i)(index+1:index+1)
                    else if (iachar(banks(i)(index:index)) > iachar(second_char)) then
                        second_char = banks(i)(index:index)
                    end if
                end do

                count = count + (iachar(first_char) - iachar('0')) * 10 + iachar(second_char) - iachar('0')

            end block
        end do

    end function part1

    function part2(banks) result(count)
        implicit none
        character(len=100), dimension(:), intent(in) :: banks
        integer*8 :: count, i, bank_len
        integer*8 :: digit, found_index, current_index
        character :: current_char
        bank_len = len_trim(banks(1))

        count = 0

        do i = 1, size(banks)
            block
                found_index = 1

                ! Calculate each possible digit
                do digit = 1, 12
                    current_char = banks(i)(found_index:found_index)
                    found_index = found_index + 1
                    ! Start from where we found our previous digit
                    do current_index = found_index, bank_len - 12 + digit
                        ! If its bigger then pick it!
                        if (iachar(banks(i)(current_index:current_index)) > iachar(current_char)) then
                            current_char = banks(i)(current_index:current_index)
                            found_index = current_index + 1
                        end if
                    end do
                    ! Convert char to a number and then scale it to the digit position it should be in and add it to our total
                    count = count + (((iachar(current_char) - iachar('0'))) * (10 ** (12 - digit)))
                end do
            end block
        end do
    end function part2

end program day3
