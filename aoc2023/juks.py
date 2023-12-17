import fileinput, heapq

g = { ( x, y ): int( c )
      for y, r in enumerate( fileinput.input() )
      for x, c in enumerate( r.strip( '\n' ) ) }
w = max( x for x, y in g.keys() ) + 1
h = max( y for x, y in g.keys() ) + 1

q = [ ( 0, 0, 0, 0, 0, 0 ) ]
b = { n[ 1 : ]: 0 for n in q }
heapq.heapify( q )
while q:
    c, x, y, xi, yi, s = heapq.heappop( q )
    if ( x, y ) == ( w - 1, h - 1 ) and s >= 4:
        print( c )
        break
    for nxi, nyi in ( ( 1, 0 ), ( -1, 0 ), ( 0, -1 ), ( 0, 1 ) ):
        nx, ny = x + nxi, y + nyi
        ns = ( s + 1 ) if ( nxi, nyi ) == ( xi, yi ) else 1
        nc = c + g.get( ( nx, ny ), 1e30 )
        n = ( nc, nx, ny, nxi, nyi, ns )
        if ( ( xi, yi ) != ( -nxi, -nyi ) and
             ( s >= 4 or ( xi, yi ) == ( nxi, nyi ) or ( xi, yi ) == ( 0, 0 ) ) and
             ( s < 10 or ( xi * nxi + yi * nyi ) == 0 ) and
             0 <= nx < w and 0 <= ny < h ):
            if nc < b.get( n[ 1 : ], 1e30 ):
                heapq.heappush( q, n )
                b[ n[ 1 : ] ] = nc

