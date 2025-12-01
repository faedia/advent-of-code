module day1_mod
    implicit none
    public part1, part2

contains
    function part1(operations) result(count)
        implicit none
        integer, dimension(:), intent(in) :: operations
        integer :: count, current, i

        current = 50
        count = 0
        
        do i = 1, size(operations)
            ! This winds us back round to the start
            current = mod(current + operations(i) + 100, 100)
            if (current == 0) then
                count = count + 1
            end if
        end do

    end function part1

    function part2(operations) result(count)
        implicit none
        integer, dimension(:), intent(in) :: operations
        integer count, current, i

        current = 50
        count = 0

        do i = 1, size(operations)
            block
                integer :: offset
                ! This counts how many times we have seen 0 when applying the operation.
                ! NOTE: This also applies if you start at 0 and have a negative operation.
                !    So we need to make sure we subtract 1 in that scenario.
                ! Notably this works by normalising our current value to -50 to 49.
                ! We then add our operation to the value and abs it, This now gives us values >0
                ! where each 100 over 50 indicates a winding.
                offset = ((abs((current - 50) + operations(i)) + 50) / 100)
                if (current == 0 .and. operations(i) < 0) then
                    offset = offset - 1
                end if
                count = count + offset
                current = mod(current + mod(operations(i), 100) + 100, 100)
            end block
        end do
    end function part2


end module day1_mod


program day1
    use day1_mod

    implicit none
    integer, dimension(:), allocatable :: operations
    integer :: part1_result, part2_result
    integer :: io

    open(newunit=io, file="day1.txt", status="old", action="read")
    ! The following bit of magic finds how big a file is and then allocates
    ! that many elements into our array.
    block
        integer :: ios, num_lines
        num_lines = 0
        do 
            read(io, *, iostat=ios)
            if (ios /= 0) exit
            num_lines = num_lines + 1
        end do
        rewind(io)

        allocate(operations(num_lines))
    end block

    block
        integer :: i, value
        character :: direction

        do i = 1, size(operations)
            ! not really 100% sure what '(A1,I10)' is doing??
            ! I think it means, read a single array value of type direction
            ! and then read an integer as base10
            read(io, '(A1,I10)') direction, value
            ! We are going to treat left-wards as negative offsets
            if (direction == 'L') then
                value = -value
            end if
            operations(i) = value
        end do
    end block


    part1_result = part1(operations)

    print *, part1_result

    part2_result = part2(operations)

    print *, part2_result

end program day1
