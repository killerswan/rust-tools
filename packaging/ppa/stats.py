# originally from jtaylor: http://paste.ubuntu.com/821868/
from collections import defaultdict

def getDownloadCounts(ppa, **kwargs):
  """ binary_name, created_since_date, exact_match, version, distro_arch_series"""

  stat = defaultdict(int)
  ppa  = lp.me.getPPAByName(name=ppa)
  pub  = ppa.getPublishedBinaries(**kwargs)

  for binary in pub:
    cc = binary.getDownloadCount()

    if not binary.architecture_specific:
      stat[binary.binary_package_name] = 0

    stat[binary.binary_package_name] += cc

    print binary.binary_package_name, binary.binary_package_version, binary.distro_arch_series, cc

    print binary.getDailyDownloadTotals(start_date='2012-01-01', end_date='2012-10-01')

    print ""

  return stat

